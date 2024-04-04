use std::{fs::File, io::Read, path::PathBuf};

use data_encoding::BASE32;
use serde::{Deserialize, Deserializer, Serialize};
use sha1::{Sha1, Digest};
use urlencoding::encode;

/// Parsed torrent file
#[derive(Debug, Deserialize, Serialize)]
pub struct Torrent {
    announce: String,
    #[serde(rename(deserialize = "announce-list"))]
    announce_list: Option<Vec<Vec<String>>>,
    info: Info
}


#[derive(Debug, Deserialize, Serialize)]
struct TorrentRaw {
    announce: String,
    #[serde(deserialize_with = "from_hex")]
    info: Vec<u8>
}

/// Torrent infos
#[derive(Debug, Deserialize, Serialize)]
struct Info {
    length: Option<u64>,
    name: Option<String>,
    #[serde(rename(deserialize = "piece length"))]
    piece_length: u64,
    #[serde(deserialize_with = "from_hex")]
    pieces: Vec<u8>,
    private: Option<u32>
}

/// Object containing all necessary information to build a magnet uri
#[derive(Debug, Deserialize, Serialize)]
pub struct MagnetDetail {
    pub torrent: Torrent,
    pub hash: String

}
impl MagnetDetail {

    /// Transform a magnet detail into a magnet URI
    pub fn as_magnet(&self) -> String {
        let mut params: Vec<(&str,String)> = vec![];
        if let Some(name) = &self.torrent.info.name {
            params.push(("dn", encode(&name).to_string()));
        }
        if let Some(length) = &self.torrent.info.length {
            params.push(("xl", length.to_string()));
        }
        if let Some(list) = &self.torrent.announce_list {
            for node in list {
                for announce in node {
                    params.push(("tr", encode(&announce).to_string()));
                }
            }
        } else {
            params.push(("tr", encode(&self.torrent.announce).to_string()));
        }
        let params_string = params.into_iter().map(|(key, value)| format!("{}={}", key, value)).collect::<Vec<_>>().join("&");
        
        format!("magnet:?xt=urn:btih:{}&{}", self.hash.clone(), params_string)
    }
}

fn from_hex<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &[u8] = Deserialize::deserialize(deserializer)?;
    // do better hex decoding than this
    Ok(s.to_vec())
}

pub fn decode_file(path: PathBuf) -> MagnetDetail {
    let mut file = File::open(path).unwrap();
    let mut encoded = Vec::new();
    file.read_to_end(&mut encoded).unwrap();
    decode(encoded)
}


pub fn decode(encoded: Vec<u8>) -> MagnetDetail {
    let info: Torrent = bt_bencode::from_slice(&encoded).unwrap();

    let info_raw: TorrentRaw = bt_bencode::from_slice(&encoded).unwrap();

    let mut hasher = Sha1::new();
    hasher.update(info_raw.info);
    let result = hasher.finalize();
    let r = BASE32.encode(&result);

    MagnetDetail {
        torrent: info,
        hash: r
    }
}

/// Get magnet from a torrent file
pub fn magnet_from_torrent_file(path: PathBuf) -> String {
    let infos = decode_file(path);
    infos.as_magnet()
}

/// Get magnet from a torrent buffer
pub fn magnet_from_torrent(data: Vec<u8>) -> String {
    let infos = decode(data);
    infos.as_magnet()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn parse_file() {
        let path = PathBuf::from_str("test_data/big-buck-bunny.torrent").unwrap();
        let result = magnet_from_torrent_file(path);
        assert_eq!(result, "magnet:?xt=urn:btih:3WBFL3G4PSSV7MF37AJSHWDQMLNR63I4&dn=Big%20Buck%20Bunny&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&tr=wss%3A%2F%2Ftracker.fastcast.nz");
    }

    #[test]
    fn parse_data() {
        let mut file = File::open("test_data/big-buck-bunny.torrent").unwrap();
        let mut encoded = Vec::new();
        file.read_to_end(&mut encoded).unwrap();

        let result = magnet_from_torrent(encoded);
        assert_eq!(result, "magnet:?xt=urn:btih:3WBFL3G4PSSV7MF37AJSHWDQMLNR63I4&dn=Big%20Buck%20Bunny&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&tr=wss%3A%2F%2Ftracker.fastcast.nz");
    }

    #[test]
    fn parse_torrent() {
        let path = PathBuf::from_str("test_data/big-buck-bunny.torrent").unwrap();
        let result = decode_file(path);
        assert_eq!(result.hash, "3WBFL3G4PSSV7MF37AJSHWDQMLNR63I4");
        assert_eq!(result.torrent.info.name, Some("Big Buck Bunny".to_owned()));
    }
}

