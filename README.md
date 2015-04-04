# machete
Builds a graph of how C# classes are related to each other. I was hoping to find a way to meaningfully visualize these relationships. But I ran into the problem that these graphs look like tangled messes, crowded by the edges coming out of few frequently used classes.

Usage: machete \<path\>

The path is recursively searched for C# source files, from which a graph of class relationships is built and spat out in DOT format to stdout.

Disclaimer: This is hacky and it will draw relationships where they shouldn't exist (hah, what's namespacing?).
