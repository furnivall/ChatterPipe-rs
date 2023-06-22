# ChatterPipe-rs
ChatterPipe is a CLI tool that lets you interact with powerful language models like GPT-4 for single shot prompting using text files and temporary/permanently configurable parent prompts. Just provide a text file as input and let ChatterPipe do its magic!

## Installation
Install ChatterPipe using cargo:
`cargo install chatterpipe`

## Setup/Configuration
Before using ChatterPipe for the first time (or at any time of your choice), run the setup command to configure your custom parent prompt:
`ctp setup`

The default prompt is as follows: 
```
Summarise the following in 300 tokens or less. Give your best attempt
```

You can also set a custom parent prompt for a single command using the `-p` option:
`ctp <text_file_path> -p "Your custom parent prompt"`

An example of a possible use case for this would be something along the lines of:
`ctp <yaml file> -p "lint this file for me"`

To view the current parent prompt, run:
`ctp current`

## Usage
Run ChatterPipe with the following command:
```ctp <text_file_path> [--engine <engine>] [--raw]```
Where:
- <text_file_path> is the path to the text file you want to summarize.
- <engine> is an optional parameter specifying the language model to use (default is "gpt-4"). Available options are:
- "g4" for GPT-4
- "g4-32" for GPT-4-32k
- "g3" for GPT-3.5-turbo
- `--raw` is an optional flag to print the raw API response. Useful for debugging.

### Example
```
ctp myfile.txt --engine g4
```

### *Important:* Make sure to set your OpenAI API key as an environment variable before running ChatterPipe. 
For example:
```
export OPENAI_API_KEY=<your_api_key_here>
```

Don't have an API key? Sign up at https://beta.openai.com/signup/

For more information on the OpenAI API, visit https://beta.openai.com/docs/

## License

This project is licensed under the MIT License. See the LICENSE file for details.

Happy summarizing!

