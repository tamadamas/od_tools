# Log generator for [OpenDominion Game](https://github.com/OpenDominion/OpenDominion)

Converted from the script included in sim file from [Yami-10/OD-Simulator](https://github.com/Yami-10/OD-Simulator)

# The idea

I work on Linux so don't have Excel and has no ability to generate `log.txt` from sim file.

This tool writen in [GO](https://go.dev/) can generate `log.txt` from Excel file
both local and downloaded from Google Sheet. It should work in any Operation System.

The code contains only final script generation logic. All other things left the same in Excel file.

Remember to open Excel sim before generation. ALl formulas should run and update their values with Excel.

# Get binary

You can use prebuilt binaries at [Releases](https://github.com/tamadamas/od_tools/releases) or build from sources with

To install `go` check [instructions here](https://go.dev/doc/install)

and then run

```
go install github.com/tamadamas/od_tools/cmd/sim@latest
```

If `@latest` does not work, try to include specific version as `@v1.0.2-sim`

it installs to `GOPATH/bin` with a name `sim`.

Rename it whatever you like.

Make sure `GOPATH` is visible to the system. [This guide](https://go.dev/wiki/SettingGOPATH) will help you.

# Usage

Next will work for Linux and Mac

```
sim generate_log -sim OpenDominionSim.xlsm -result sim.txt
```

For windows you can also run `sim` from terminal or put command line to the exe options.

I don't have Windows and can't test and describe the actual process, it would be helpfull if someone describe that and make a pull request ^\_^

Get file from [Yami-10/OD-Simulator](https://github.com/Yami-10/OD-Simulator)

# Bug reports

If you see any issues or want an improvement, feel free to create an issue and describe the problem.

Write me in Discord @tomas_tamadamas
