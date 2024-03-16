# Unity Unpacker
Unity Unpacker is a command line program designed to unpack '.unitypackage' files. 

## Usage
Unity unpacker requires at least one parameter, the file name. So running the programm with 
```
./unity_unpacker -f test.unitypackage
```
This command would extract the file test.unitypackage in the current working directory. The path the tool extracts to is
the name of the package (test in this example), inside the current working directory.

However, using the 'dir' parameter the target directory can be passed to the tool. On top of that the 'tmp_dir' parameter can be used to set the directory the package is unpacked to before copying the assets files over to the target directory.
```
./unity_unpacker -f assets/test.unitypackage -d assets/test -t assets/tmp
```

All directories the tool needs to unpack the package will be created.

## Assets
All assets are from https://opengameart.org and are licenced as CC0.
