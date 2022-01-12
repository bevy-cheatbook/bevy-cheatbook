# Support for loading different file formats

By default, only a few asset file formats are enabled:
 - Images: PNG and HDR
 - Audio: OGG/Vorbis

You can enable more formats with cargo features:
 - Images: JPEG, TGA, BMP, DDS
 - Audio: FLAC, MP3, WAV

```toml
[dependencies.bevy]
version = "0.6"
features = ["jpeg", "tga", "bmp", "dds", "flac", "mp3", "wav"]
```
