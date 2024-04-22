from sc2.constants import *

py_list = [UnitTypeId.SCV, UnitTypeId.DRONE]

rust_list = ""

print("const X : &'static [UnitTypeId] = &[")
for unit_type in py_list :
    rust_list = ""
    rust_list += "UnitTypeId::"
    rust_list += (str(unit_type.name))
    rust_list += ", "
    print(rust_list)
print("];")
