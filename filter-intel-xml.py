#!/usr/bin/python
import sys
import xml.etree.ElementTree as xml

SKIPZERO = True
COLORS = True

"""
A script used to filter intel intrinsics XML and print out instructions
related with them
In case of problems with ansi colors on Windows, switch COLORS to False
Also, it uses \b character hack to "truncate" strings
(while printing \b moves cursor one character back, so it appears as truncated)

Usage: ./filter-intel-xml.py TECH?

Examples:
./filter-intel-xml.py AVX2 # filters avx2 instructions
./filter-intel-xml.py # prints all intrinsics

Setting SKIPZERO means this script won't print intrinsics with no assembly
instructions related with them.
This may happen because the XML may not be 100% completed, but also most of
casts just change type and do not touch the bits

Link to the XML: 
https://software.intel.com/sites/landingpage/IntrinsicsGuide/files/data-3.5.0.xml
(beware, for a text file, its huge)
"""

CBLUE = ''
CGREEN = ''
CEND = ''
CBEIGE = ''

if COLORS:
    CBLUE = '\033[94m'
    CGREEN = '\033[92m'
    CEND = '\x1b[0m'
    CBEIGE  = '\33[36m'

tech = ''
filename = ''
if len(sys.argv) >= 2:
    filename = sys.argv[1]
else:
    print("Not enough arguments")
    exit(1)

if len(sys.argv) == 3:
    tech = sys.argv[2]

tree = xml.parse(filename)

for child in tree.getroot():
    if len(tech) != 0 and child.attrib["tech"] != tech:
        continue

    instructions = child.findall("instruction")
    instr_no = len(instructions)

    intrinsic_name = child.attrib["name"]
    intrinsic_name += '('
    for arg in child.findall("parameter"):
        typ = arg.attrib["type"]
        name = arg.attrib.get("varname", "\b")
        intrinsic_name += f"{CBEIGE}{typ}{CBLUE} {name}, "
    intrinsic_name = intrinsic_name[:-2] + ')'

    if instr_no == 0 and SKIPZERO:
        continue
    elif instr_no == 1:
        print(f"{CBLUE}{intrinsic_name}{CEND}", end=': ')
    else:
        print(f"{CBLUE}{intrinsic_name}{CEND}")

    for instr in instructions:
        attribs = instr.attrib
        name = attribs["name"].lower()
        form = attribs.get("form", "\b")
        print(f"{CGREEN}{name}{CEND} {form}")

