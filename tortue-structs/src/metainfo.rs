use serde::{
    de::{Error, MapAccess, Unexpected, Visitor},
    Deserialize, Serialize,
};
use tortue_bencode::{de::Deserializer, from_value, BencodedValue};

/// All data in a metainfo file is bencoded. The specification for bencoding is defined above.
///
/// The content of a metainfo file (the file ending in ".torrent") is a bencoded dictionary, containing the keys listed below.
/// All character string values are UTF-8 encoded.
///
/// **⚠ Note that this uses a lifetime to do zero copy deserialization**
///
/// [source](https://wiki.theory.org/index.php/BitTorrentSpecification#Identification)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Metainfo<'a> {
    /// The announce URL of the tracker
    pub announce: &'a str,

    /// This is an extention to the official specification, offering backwards-compatibility.
    ///
    /// The official request for a specification change is [here](http://bittorrent.org/beps/bep_0012.html).
    #[serde(rename = "announce-list")]
    pub announce_list: Option<Vec<Vec<&'a str>>>,

    /// The creation time of the torrent, in standard UNIX epoch format (seconds since 1-Jan-1970 00:00:00 UTC)
    #[serde(rename = "creation date")]
    pub creation_date: Option<i64>,

    /// Free-form textual comments of the author
    pub comment: Option<&'a str>,

    /// Name and version of the program used to create the .torrent
    #[serde(rename = "created by")]
    pub created_by: Option<&'a str>,

    /// The string encoding format used to generate the **pieces** part of the **info** dictionary in the .torrent metafile
    pub encoding: Option<&'a str>,

    pub info: Info<'a>,
}

/// This is the section of the metainfo file that contains information about the file
/// or files being transferred
///
/// **⚠ Note that this uses a lifetime to do zero copy deserialization**
///
/// [source](https://wiki.theory.org/index.php/BitTorrentSpecification#Identification)
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub enum Info<'a> {
    /// The torrent contains a single file
    SingleFile {
        /// Number of bytes in each piece
        #[serde(rename = "piece length")]
        piece_length: i64,

        /// 20 bytes SHA-1 hash value, one per piece
        #[serde(with = "serde_bytes")]
        pieces: &'a [u8],

        /// if `Some(true)` the client MUST publish its presence to get other
        /// peers ONLY via the trackers explicitly described in the metainfo file.
        /// If this field is set to None or Some(false), the client may obtain peer
        /// from other means, e.g. PEX peer exchange, dht. Here, "private" may be
        /// read as "no external peer source".
        private: Option<bool>,

        /// See the structure description for its fields, not that it is flattened!
        #[serde(flatten)]
        info: FileInfo<'a>,
    },

    /// The torrent contains multiple files
    MultiFile {
        #[serde(rename = "piece length")]
        piece_length: i64,

        #[serde(with = "serde_bytes")]
        pieces: &'a [u8],

        private: Option<bool>,

        #[serde(rename = "name")]
        dir_name: &'a str,

        files: Vec<FileInfo<'a>>,
    },
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct FileInfo<'a> {
    #[serde(rename = "name")]
    file_name: &'a str,

    #[serde(rename = "length")]
    file_size: i64,

    #[serde(with = "serde_bytes")]
    md5sum: Option<&'a [u8]>,
}

impl<'a> Info<'a> {
    pub fn is_single_file(&self) -> bool {
        match self {
            Info::SingleFile { .. } => true,
            _ => false,
        }
    }

    pub fn is_multi_file(&self) -> bool {
        !self.is_single_file()
    }
}

struct FileInfoVisitor;

impl<'de> Visitor<'de> for FileInfoVisitor {
    type Value = Info<'de>;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str("map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut pieces_length = None;
        let mut pieces = None;
        let mut private = None;

        let mut name = None;
        let mut files_size = None;
        let mut md5sum = None;

        let mut files = None;

        while let Some((k, v)) =
            map.next_entry::<String, BencodedValue<'de>>()?
        {
            match &k as &str {
                "piece length" => {
                    pieces_length.replace(from_value::<i64>(v).map_err(
                        |_e| {
                            Error::invalid_type(
                                Unexpected::Other("not an i64"),
                                &self,
                            )
                        },
                    )?);
                }
                "pieces" => {
                    pieces.replace(
                        serde_bytes::deserialize(Deserializer::from_value(v))
                            .map_err(|_e| {
                                Error::invalid_type(
                                    Unexpected::Other("not a byte array"),
                                    &self,
                                )
                            })?,
                    );
                }
                "private" => {
                    private.replace(from_value::<bool>(v).map_err(|_e| {
                        Error::invalid_type(
                            Unexpected::Other("not a bool"),
                            &self,
                        )
                    })?);
                }
                "name" => {
                    name.replace(from_value::<&'de str>(v).map_err(|_e| {
                        Error::invalid_type(
                            Unexpected::Other("not a string"),
                            &self,
                        )
                    })?);
                }
                "length" => {
                    files_size.replace(from_value::<i64>(v).map_err(|_e| {
                        Error::invalid_type(
                            Unexpected::Other("not an i64"),
                            &self,
                        )
                    })?);
                }
                "md5sum" => {
                    md5sum.replace(from_value::<&'de [u8]>(v).map_err(
                        |_e| {
                            Error::invalid_type(
                                Unexpected::Other("not an md5"),
                                &self,
                            )
                        },
                    )?);
                }
                "files" => {
                    files.replace(
                        from_value::<Vec<FileInfo<'de>>>(v).map_err(|_e| {
                            Error::invalid_type(
                                Unexpected::Other("not a list of FileInfo"),
                                &self,
                            )
                        })?,
                    );
                }
                key => {
                    return Err(Error::unknown_field(
                        key,
                        &[
                            "piece length",
                            "pieces",
                            "private",
                            "name",
                            "length",
                            "md5sum",
                            "files",
                        ],
                    ))
                }
            }
        }

        if pieces.is_none() {
            return Err(Error::missing_field("pieces"));
        }

        if name.is_none() {
            return Err(Error::missing_field("name"));
        }

        if pieces_length.is_none() {
            return Err(Error::missing_field("pieces length"));
        }

        if files.is_none() {
            if files_size.is_none() {
                return Err(Error::missing_field("length"));
            }

            Ok(Info::SingleFile {
                piece_length: pieces_length.unwrap(),
                pieces: pieces.unwrap(),
                private,
                info: FileInfo {
                    file_name: name.unwrap(),
                    file_size: files_size.unwrap(),
                    md5sum,
                },
            })
        } else {
            Ok(Info::MultiFile {
                piece_length: pieces_length.unwrap(),
                pieces: pieces.unwrap(),
                private,
                dir_name: name.unwrap(),
                files: files.unwrap(),
            })
        }
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Info<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(FileInfoVisitor)
    }
}

#[cfg(test)]
mod simple_test {
    use crate::Metainfo;
    use tortue_bencode::from_bytes;
    #[test]
    fn deserialize_single_file() {
        let single_file = b"d8:announce11:example.com4:infod12:piece lengthi4e6:pieces4:\x01\x02\x03\x044:name5:hello6:lengthi64e6:md5sum32:\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x30\x31\x32ee";

        if let Ok(val) = from_bytes::<Metainfo>(single_file) {
            assert!(val.info.is_single_file());
        } else {
            assert!(false, "could not deserialize matainfo");
        }
    }

    #[test]
    fn deserialize_multi_file() {
        let multi_file = b"d8:announce11:example.com4:infod12:piece lengthi4e6:pieces4:\x01\x02\x03\x044:name5:hello5:filesld4:name5:world6:lengthi64e6:md5sum32:\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x30\x31\x32eeee";

        if let Ok(val) = from_bytes::<Metainfo>(multi_file) {
            assert!(val.info.is_multi_file());
        } else {
            assert!(false, "could not deserialize matainfo");
        }
    }
}
