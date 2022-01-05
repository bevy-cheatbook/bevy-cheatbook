# Support for loading different file formats

By default, only a few asset file formats are enabled:
 - Images: PNG and HDR
 - Audio: OGG/Vorbis

You can enable more formats with cargo features:
 - Images: JPEG, TGA, BMP, DDS
 - Audio: FLAC, OGG/Vorbis, WAV

```toml
[dependencies.bevy]
version = "0.6"
features = ["jpeg", "tga", "bmp", "dds", "flac", "vorbis", "wav"]
```
