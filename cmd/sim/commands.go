package main

import (
	"flag"
	"fmt"
	"os"
)

type FlagSetVars struct {
	debugEnabled bool
	simPath      string
	resultPath   string
	logPath      string
	hour         int
}

const (
	GenerateLogCmd = "generate_log"
	ParseLogCmd    = "parse_log"
)

func (c *FlagSetVars) GenerateLogCmd() *flag.FlagSet {
	cmd := flag.NewFlagSet(GenerateLogCmd, flag.ExitOnError)
	// cmd.BoolVar(&c.debugEnabled, "debug", false, "Enable debug logging")
	cmd.StringVar(&c.simPath, "sim", "", "Path to the sim file")
	cmd.StringVar(&c.resultPath, "result", "", "Path to the result file \"\" or \"std\" prints to stdout")
	cmd.IntVar(&c.hour, "hour", 0, "Set current hour")
	cmd.Usage = func() {
		fmt.Printf("Usage of %s %s:\n", os.Args[0], GenerateLogCmd)
		cmd.PrintDefaults()
		fmt.Println("Example:")
		fmt.Printf("  %s %s -sim sim.xlsm -result sim.txt\n\n", os.Args[0], GenerateLogCmd)
	}

	return cmd
}

func (c *FlagSetVars) ParseLogCmd() *flag.FlagSet {
	cmd := flag.NewFlagSet(ParseLogCmd, flag.ExitOnError)
	// cmd.BoolVar(&c.debugEnabled, "debug", false, "Enable debug logging")
	cmd.StringVar(&c.logPath, "log", "", "Path to the txt log file")
	cmd.Usage = func() {
		fmt.Printf("Usage of %s %s:\n", os.Args[0], ParseLogCmd)
		cmd.PrintDefaults()
		fmt.Println("\nExample:")
		fmt.Printf("  %s %s -log sim.txt\n", os.Args[0], ParseLogCmd)
	}

	return cmd
}
