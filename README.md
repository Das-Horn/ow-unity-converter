# Overwatch Texture Conversion Tool
[![GitHub issues](https://img.shields.io/github/issues/Das-Horn/ow-unity-converter?style=for-the-badge)](https://github.com/Das-Horn/ow-unity-converter/issues) ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Das-Horn/ow-unity-converter/Rust?style=for-the-badge)

This is a tool that allows you to convert the compressed PBR textures from Overwatch into a format that is compatible with unity's URP PBR render pipeline.

## NFD

Used the nfd crate from crates.io to add file dialog for easier use of the tool. This can be accessed by calling the executable with the -s argument.

### Usage:
```
ow-unity-converter [-s] "File-Path" ... 
```

The utility will save the output image in the same path as the original image and append "-" to the end of the file name.

## TODO
- <s>Port image processing from Python script to rust & include the original script.</s>
- <s>Add argument support to supply file path from command line.</s>
- Implement Albedo masking from mask.
- <s>Look into easier methods to add paths.</s>
- <s>Batch processing.</s>
