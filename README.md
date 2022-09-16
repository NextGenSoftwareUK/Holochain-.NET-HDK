# Holochain-.NET-HDK
I have a strong .NET (C#) background and have created a .NET Client called HoloNET, which has been gifted forward to this amazing community:

https://github.com/holochain-open-dev/holochain-client-csharp

I now REALLY want to create a .NET HDK. I would welcome people to join. 

.NET can compile to WASM so I know this is possible. If we can get this done I think a lot of people would find it much easier to create holochain apps as well as it opening up holochain to a much bigger audience and allow many more advanced applications of it. .NET allows us to build apps/games for Unity, both of which have a massive ecosystem and will help turbo charge the holochain ecosystem. This is why I am creating my Our World game with holochain, .NET & Unity. Holochain already talks to .NET and Unity through my HoloNET, but now I wish to help speed up the holochain hApp dev by creating a .NET HDK. Any tips to help with this would be really appreciated, I will create a Open Source repro where we can all contribute... many thanks! 

The first steps are what is required to create a HDK for Holochain? What are the API calls that need to be wrapped?

The current codebase here is a .NET HDK Low Code Generator that can generate dynamic C# and Rust code from meta data. The C# wraps around HoloNET to make calls into your Rust hApp (from the generated rust code). The Rust code acts as the DAL (Data Access Layer) and the C# code acts as the BLL (Business Logic layer).

STAR ODK Omniverse Metaverse Interoperable Low Code Generator evolved from this original .NET HDK code. The plan is for them to share the same core templating engine and whereas the .NET HDK will be tailored more for just Holochain, STAR also connects to the OASIS API and allows metaverse objects such as planets, moons, stars, etc to be generated.

Check out the main repo for more info:
https://github.com/NextGenSoftwareUK/Our-World-OASIS-API-HoloNET-HoloUnity-And-.NET-HDK
