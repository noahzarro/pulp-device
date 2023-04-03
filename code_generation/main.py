import csv

# formatted like 'nr,name,doc'

rust_str = ""
link_str = ""
link_defaults_str = ""

with open('interrupts.csv', 'r') as file:
    reader = csv.reader(file)
    rust_str = "pub enum Interrupt {\n"
    for row in reader:
        nr = row[0]
        name = row[1]
        doc = row[2]
        rust_str += '#[doc = "{nr} - {doc}"]\n{name} = {nr},\n'.format(nr=nr, name=name, doc=doc)
        link_str += 'PROVIDE(int_{nr} = {name});\n'.format(nr=nr, name=name)
        link_defaults_str += 'PROVIDE({name} = DefaultHandler);\n'.format(nr=nr, name=name)
    rust_str += "}\n"
    
with open("int_link.x", "w") as file:
    file.write(link_str)
    file.write("\n")
    file.write(link_defaults_str)
    
with open("rust.rs", "w") as file:
    file.write(rust_str)