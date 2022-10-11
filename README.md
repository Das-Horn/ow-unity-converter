# Overwatch Texture Conversion Tool
[![GitHub issues](https://img.shields.io/github/issues/Das-Horn/ow-unity-converter?style=for-the-badge)](https://github.com/Das-Horn/ow-unity-converter/issues) ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Das-Horn/ow-unity-converter/Rust?style=for-the-badge)
This is a tool that allows you to convert the compressed PBR textures from Overwatch into a format that is compatible with unity's URP PBR render pipeline.

## NFD

Did a simple implementation of the nfd library for rust to allow image selection from a file dialog. This should work nativly on all Operating Systems and can be accessed via using -s instead of providing paths.

### Usage:
```
command "File-Path" ...
```

The utility will save the output image in the same path as the original image and append "-" to the end of the file name.

## TODO
- <s>Port image processing from Python script to rust & include the original script.</s>
- <s>Add argument support to supply file path from command line.</s>
- Implement Albedo masking from mask.
- <s>Look into easier methods to add paths.</s>
- <s>Batch processing.</s>