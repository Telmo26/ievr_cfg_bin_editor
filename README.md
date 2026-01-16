# IEVR cfg.bin Editor

This project stems mostly from the need to read and write the configuration files for Inazuma Eleven Victory Road. The exisiting library is written entirely in C#, making it very hard to integrate into Rust projects. This project is an attempt at reimplementing the logic of [CfgBinEditor](https://github.com/onepiecefreak3/CfgBinEditor) by (onepiecefreak3)[https://github.com/onepiecefreak3]. 

# Features

Currently none, so here are the goals of this project :
- Create a full RDBN and T2B parser to be able to read the config files
- Create a full write for those to enable modification
- Create an FFI interface to make the library usable by any program that can link C code
- Create  a (nicer) GUI to make editing the files easier
- Integrate the same system of IDs and tags to enable cross-compatibility with [CfgBinEditor](https://github.com/onepiecefreak3/CfgBinEditor)