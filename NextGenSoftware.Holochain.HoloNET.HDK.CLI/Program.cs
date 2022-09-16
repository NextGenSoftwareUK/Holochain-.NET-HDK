﻿using NextGenSoftware.Holochain.HoloNET.HDK.Core;
using System;
using System.Reflection;

namespace NextGenSoftware.Holochain.HoloNET.HDK.CLI
{
    class Program
    {
        static void Main(string[] args)
        {
            if (args.Length == 0)
            {
                var versionString = Assembly.GetEntryAssembly()
                                        .GetCustomAttribute<AssemblyInformationalVersionAttribute>()
                                        .InformationalVersion
                                        .ToString();

                Console.WriteLine($"***********************************************");
                Console.WriteLine($"NextGen Software Holochain HoloNET HDK CLI v{versionString}");
                Console.WriteLine($"***********************************************");
                Console.WriteLine("\nUsage:");
                Console.WriteLine("  star --light -classFolder");
                Console.WriteLine("  star --convert -rusthAppRootFolder");
                Console.WriteLine($"***********************************************");
                return;
            }


            if (args[0].ToLower() == "light")
                Build(args[1]);


           // NextGenSoftware.Holochain.HoloNET.HDK.Core.CSharpTemplates. _ssss = new SuperZome("http://www.localhost:8888", HolochainBaseZome.HoloNETClientType.Desktop);
            //  _ssss.
       
    }

        static void Build(string classFolder)
        {
            Star.Light(GenesisType.Planet, "Our World", classFolder);
        }
    }
}
