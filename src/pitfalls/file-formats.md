# Support for loading different file formats

By default, only a few asset file formats are enabled:
 - Images: PNG and HDR
 - Audio: MP3

You can enable more formats with cargo features:
 - Images: JPEG, TGA, BMP, DDS
 - Audio: FLAC, OGG, WAV

```toml
[dependencies.bevy]
version = "0.4"
features = ["jpeg", "tga", "bmp", "dds", "flac", "ogg", "wav"]
```
