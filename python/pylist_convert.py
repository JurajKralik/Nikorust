from sc2.constants import *

py_list = [UnitTypeId.DARKTEMPLAR, UnitTypeId.MOTHERSHIP, UnitTypeId.BANSHEE, UnitTypeId.GHOST, UnitTypeId.WIDOWMINE, UnitTypeId.WIDOWMINEBURROWED, UnitTypeId.LURKER, UnitTypeId.LURKERMP, UnitTypeId.LURKERBURROWED, UnitTypeId.LURKERMPBURROWED, UnitTypeId.ROACHBURROWED]

rust_list = ""

print("const X : &'static [UnitTypeId] = &[")
for unit_type in py_list :
    rust_list = ""
    rust_list += "UnitTypeId::"
    rust_list += (str(unit_type.name))
    rust_list += ", "
    print(rust_list)
print("];")
