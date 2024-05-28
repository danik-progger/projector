package main

import (
	"encoding/json"
	"fmt"
	"log"

	"github.com/danik-progger/pkg/projector"
)

func main() {
	opts, err := projector.GetOpts()
	if err != nil {
		log.Fatal(err)
	}

	config, err := projector.NewConfig(opts)
	if err != nil {
		log.Fatal("Unable to get config", err)
	}

	proj := projector.NewProjector(config)
	if config.Operation == projector.Print {
		if len(config.Args) > 0 {
			value, ok := proj.GetValue(config.Args[0])
			if ok {
				fmt.Printf("%v\n", value)
			} else {
				fmt.Print("Nothing found\n")
			}
		} else {
			data := proj.GetValueAll()
			jsonStr, err := json.Marshal(data)
			if err != nil {
				fmt.Errorf("this line should never been reached %v\n", err)
			}
			fmt.Printf("%v\n", string(jsonStr))
		}
	}

	if config.Operation == projector.Add {
		proj.SetValue(config.Args[0], config.Args[1])
		proj.Save()	
	}

	if config.Operation == projector.Remove {
		proj.RemoveValue(config.Args[0])
		proj.Save()
	}
}
