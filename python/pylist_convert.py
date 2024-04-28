from sc2.constants import *

py_list = [UnitTypeId.HELLION, UnitTypeId.HELLIONTANK, UnitTypeId.SIEGETANK, UnitTypeId.SIEGETANKSIEGED, UnitTypeId.WIDOWMINE, UnitTypeId.WIDOWMINEBURROWED, UnitTypeId.CYCLONE, UnitTypeId.THORAP, UnitTypeId.THOR, UnitTypeId.VIKINGASSAULT, UnitTypeId.VIKINGFIGHTER, UnitTypeId.RAVEN, UnitTypeId.BANSHEE, UnitTypeId.BATTLECRUISER]

rust_list = ""

print("const X : &'static [UnitTypeId] = &[")
for unit_type in py_list :
    rust_list = ""
    rust_list += "UnitTypeId::"
    rust_list += (str(unit_type.name))
    rust_list += ", "
    print(rust_list)
print("];")
