workspace "${PROJECT_NAME}"
   configurations { "Debug", "Release" }
   architecture "x64"

project "${EXECUTABLE_NAME}"
   kind "ConsoleApp"  -- Change to 'WindowedApp' for GUI apps
   language "C++"
   cppdialect "C++17" -- Specify C++ standard
   targetdir "bin/%{cfg.buildcfg}"  -- Output directory

   files { "src/**.cpp", "src/**.h" }  -- Location of your source files

   includedirs { "include" }  -- Include directory for headers

   filter "configurations:Debug"
      defines { "DEBUG" }
      symbols "On"

   filter "configurations:Release"
      defines { "NDEBUG" }
      optimize "On"
