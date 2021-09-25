# clipper

A simple but hugely flexible command for accessing the system clipboard.

# Usage

```sh
# Print the contents of the clipboard.
clipper

# Copy the output of grep.
grep 'Name:\s[a-zA-z]+\s[a-zA-Z]+' names.txt | clipper

# Paste the contents of the clipboard into out.txt.
clipper > out.txt

# Assuming the clipboard already contains some json, format it using jq and copy it back.
clipper | jq . | clipper

# Execute a curl command, copying the output while displaying it.
curl https://google.com | clipper | cat

# Copy the contents of file.txt.
clipper file.txt

# Find the line containing the word "bazinga" in your clipboard
# copy it, then pass it to cat for display.
clipper | grep bazinga | clipper | cat

# Clear the clipboard.
clipper -x
```

# Installation


```sh
cargo install --locked --git https://github.com/insomnimus/clipper --branch main
```
