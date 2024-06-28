package helloworld

import (
  "context"
  "fmt"
  "log"
  "time"
  "io"
  "os"
  "path/filepath"

  "github.com/GoogleCloudPlatform/functions-framework-go/functions"
  "github.com/cloudevents/sdk-go/v2/event"
  "cloud.google.com/go/storage"
  "github.com/rxx/od_tools/pkg/sim"

)

func init() {
  functions.CloudEvent("HelloStorage", helloStorage)
}

// StorageObjectData contains metadata of the Cloud Storage object.
type StorageObjectData struct {
  Bucket         string    `json:"bucket,omitempty"`
  Name           string    `json:"name,omitempty"`
  Metageneration int64     `json:"metageneration,string,omitempty"`
  TimeCreated    time.Time `json:"timeCreated,omitempty"`
  Updated        time.Time `json:"updated,omitempty"`
}

// helloStorage consumes a CloudEvent message and logs details about the changed object.
func helloStorage(ctx context.Context, e event.Event) error {
  log.Printf("Event ID: %s", e.ID())
  log.Printf("Event Type: %s", e.Type())

  var data StorageObjectData
  if err := e.DataAs(&data); err != nil {
    return fmt.Errorf("event.DataAs: %v", err)
  }

  log.Printf("Bucket: %s", data.Bucket)
  log.Printf("File: %s", data.Name)
  log.Printf("Metageneration: %d", data.Metageneration)
  log.Printf("Created: %s", data.TimeCreated)
  log.Printf("Updated: %s", data.Updated)
  
  // Get storage client
	client, err := storage.NewClient(ctx)
	if err != nil {
		return fmt.Errorf("storage.NewClient: %v", err)
	}
	defer client.Close()

	rc, err := client.Bucket(data.Bucket).Object(data.Name).NewReader(ctx)
	if err != nil {
		return fmt.Errorf("Object(%q).NewReader: %v", data.Name, err)
	}
	defer rc.Close()
  
  ext := filepath.Ext(data.Name)

	inputFile, err := os.CreateTemp("", "sim-*"+ext)
	if err != nil {
		return fmt.Errorf("os.CreateTemp: %v", err)
	}
	defer inputFile.Close()

	// Copy from storage reader to the local file.
	_, err := io.Copy(inputFile, rc)
 
 if err != nil {
		return fmt.Errorf("io.Copy: %v", err)
	}
  
  if err = fileIsEmpty(inputFile); err != nil {
		return err
	}
  
  outputFile, err := os.CreateTemp("", "log-*.txt")
	if err != nil {
		return fmt.Errorf("os.CreateTemp: %v", err)
	}
	defer outputFile.Close()

	//process file
  gameLogCmd := NewGameLog(inputFile.Name(), outputFile.Name())
		gameLogCmd.Execute()
    
    if err = fileIsEmpty(inputFile); err != nil {
		  return fmt.Errorf("Log generation failed: %v", err)
	  }
    
	// Upload to output bucket
	outputBucketName := "your-output-bucket"
	outputObjectName := "log-" + data.Name 

	wc := client.Bucket(outputBucketName).Object(outputObjectName).NewWriter(ctx)
	wc.ObjectAttrs.ContentType = "text/plain"
  
	if _, err = io.Copy(wc, outputFile); err != nil {
		return fmt.Errorf("io.Copy: %v", err)
	}
	if err := wc.Close(); err != nil {
		return fmt.Errorf("Writer.Close: %v", err)
	}
	log.Printf("Blob %v uploaded to bucket %v.\n", outputObjectName, outputBucketName)
  
	// Clean up local files
	if err := os.Remove(localFile.Name()); err != nil {
		return fmt.Errorf("error removing local file: %v", err)
	}

	if err := os.Remove(outputFile.Name()); err != nil {
		return fmt.Errorf("error removing output file: %v", err)
	}

	// Remove input file from Cloud Storage
	if err := client.Bucket(data.Bucket).Object(data.Name).Delete(ctx); err != nil {
		return fmt.Errorf("error deleting object: %v", err)
	}

	return nil
}

func fileIsEmpty(file *os.File) error {
	fileInfo, err := file.Stat()
	if err != nil {
		return fmt.Errorf("error getting file info: %v", err)
	}

	if fileInfo.Size() == 0 {
		return fmt.Errorf("file is empty")
	}

	return nil
}