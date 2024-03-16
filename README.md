# Unity Unpacker
'Unity Unpacker' is a command line tool designed to unpack '.unitypackage' files. 
'.unitypackage' files are essentially gzip/tar archives that contain a directory for each asset in a unity project. The name of the directory is the guid that was assigned to the asset by the Unity Editor. Each asset has an accompanying '.meta' file that contains import information for the Unity Editor/Engine. 
What this tool does is first unpack the package into a temporary directory and after that moving the assets and the accompanying meta files over to the target directory. The tool will create all directories the package contains, so it reflects the folder structure of the package in the Unity Editor. However the extension '.meta' will be changed to '.unitymeta' since the meta extension is a bit to generic and creates conflicts with other game engines.  

## Usage
Unity unpacker requires at least one parameter, the file name. So running the programm with 
```
./unity_unpacker -f test.unitypackage
```
This command would extract the file test.unitypackage in the current working directory. The path the tool extracts to is
the name of the package (test in this example), inside the current working directory.

However, using the 'dir' parameter the target directory can be passed to the tool. On top of that the 'tmp_dir' parameter can be used to set the directory the package is unpacked to before moving the assets files over to the target directory.
```
./unity_unpacker -f assets/test.unitypackage -d assets/test -t assets/tmp
```

All directories the tool needs to unpack the package will be created.

By default the temporary directory will be deleted after all assets have been moved over to the target. However, if the 'remove_tmp' is set to false, the temp directory will remain. This can be usefull for debugging purposes.

## Assets
All assets are from https://opengameart.org and are licenced as CC0.
