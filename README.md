Parse torrent file and transform Torrent Files to a magnet URI

- Get Magnet URI from torrent file
```rust
        let path = PathBuf::from_str("test_data/big-buck-bunny.torrent").unwrap();
        let result = magnet_from_torrent_file(path);
        assert_eq!(result, "magnet:?xt=urn:btih:3WBFL3G4PSSV7MF37AJSHWDQMLNR63I4&dn=Big%20Buck%20Bunny&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&tr=wss%3A%2F%2Ftracker.fastcast.nz");
```
- Get Magnet URI from torrent raw data
```rust
        let mut file = File::open("test_data/big-buck-bunny.torrent").unwrap();
        let mut encoded = Vec::new();
        file.read_to_end(&mut encoded).unwrap();

        let result = magnet_from_torrent(encoded);
        assert_eq!(result, "magnet:?xt=urn:btih:3WBFL3G4PSSV7MF37AJSHWDQMLNR63I4&dn=Big%20Buck%20Bunny&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&tr=wss%3A%2F%2Ftracker.fastcast.nz");
```
- Get torrent details

```rust
        let path = PathBuf::from_str("test_data/big-buck-bunny.torrent").unwrap();
        let result = decode_file(path);
        assert_eq!(result.hash, "3WBFL3G4PSSV7MF37AJSHWDQMLNR63I4");
        assert_eq!(result.torrent.info.name, Some("Big Buck Bunny".to_owned()));
```