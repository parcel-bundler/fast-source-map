{
  "targets": [
    {
      "target_name": "sourcemap",
      "defines": [ "NAPI_DISABLE_CPP_EXCEPTIONS" ],
      "sources": [ "src/binding.cpp", "src/SourceMap.cpp" ],
      "include_dirs" : ["<!@(node -p \"require('node-addon-api').include\")"],
      "dependencies": ["<!(node -p \"require('node-addon-api').gyp\")"],
      'cflags!': [ '-fno-exceptions' ],
      'cflags_cc!': [ '-fno-exceptions' ],
      "conditions": [
          [ "OS=='linux'", {
              "cflags+": [ "-std=c++11", "-fexceptions" ],
              "cflags_c+": [ "-std=c++11", "-fexceptions" ],
              "cflags_cc+": [ "-std=c++11", "-fexceptions" ],
          }],
          [ "OS=='freebsd'", {
              "cflags+": [ "-std=c++11", "-fexceptions" ],
              "cflags_c+": [ "-std=c++11", "-fexceptions" ],
              "cflags_cc+": [ "-std=c++11", "-fexceptions" ],
          }],
          [ "OS=='mac'", {
              "cflags+": [ "-stdlib=libc++" ],
              "xcode_settings": {
                  "OTHER_CPLUSPLUSFLAGS" : [ "-std=c++11", "-stdlib=libc++", "-pthread" ],
                  "OTHER_LDFLAGS": [ "-stdlib=libc++" ],
                  "GCC_ENABLE_CPP_EXCEPTIONS": "YES",
                  "MACOSX_DEPLOYMENT_TARGET": "10.7",
                  "CLANG_CXX_LANGUAGE_STANDARD":"c++11",
                  "CLANG_CXX_LIBRARY": "libc++"
              },
          }],
          [
          "OS=='win'", {
            "cflags": [
              "-Wall"
            ],
            "defines": [
              "WIN"
            ],
            "msvs_settings": {
              "VCCLCompilerTool": {
                "ExceptionHandling": "2",
                "DisableSpecificWarnings": [
                  "4244"
                ],
              },
              "VCLinkerTool": {
                "LinkTimeCodeGeneration": 1,
                "OptimizeReferences": 2,
                "EnableCOMDATFolding": 2,
                "LinkIncremental": 1,
              }
            }
          }]
      ]
    }
  ]
}