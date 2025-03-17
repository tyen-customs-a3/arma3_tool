class CfgWeapons {
    class ItemCore;
    class ACE_ItemCore: ItemCore {
        class ItemInfo;
    };
    
    class FirstAidKit: ItemCore {
        type = 0;
        ACE_isMedicalItem = 1;
    };
    
    class ACE_fieldDressing: ACE_ItemCore {
        scope = 2;
        ACE_isMedicalItem = 1;
        displayName = "CSTRING(Bandage_Basic_Display)";
        descriptionShort = "CSTRING(Bandage_Basic_Desc_Short)";
    };
    
    class ACE_morphine: ACE_ItemCore {
        scope = 2;
        ACE_isMedicalItem = 1;
        displayName = "CSTRING(Morphine_Display)";
        descriptionShort = "CSTRING(Morphine_Desc_Short)";
    };
    
    class ACE_bloodIV: ACE_ItemCore {
        scope = 2;
        ACE_isMedicalItem = 1;
        displayName = "CSTRING(Blood_IV)";
        descriptionShort = "CSTRING(Blood_IV_Desc_Short)";
    };
    
    class ACE_epinephrine: ACE_ItemCore {
        scope = 2;
        ACE_isMedicalItem = 1;
        displayName = "CSTRING(Epinephrine_Display)";
        descriptionShort = "CSTRING(Epinephrine_Desc_Short)";
    };
    
    class ACE_surgicalKit: ACE_ItemCore {
        scope = 2;
        ACE_isMedicalItem = 1;
        displayName = "CSTRING(SurgicalKit_Display)";
        descriptionShort = "CSTRING(SurgicalKit_Desc_Short)";
    };
    
    class ACE_bloodIV_500: ACE_bloodIV {
        displayName = "CSTRING(Blood_IV_500)";
    };
    
    class ACE_bloodIV_250: ACE_bloodIV {
        displayName = "CSTRING(Blood_IV_250)";
    };
    
    class ACE_salineIV: ACE_ItemCore {
        scope = 2;
        ACE_isMedicalItem = 1;
        displayName = "CSTRING(Saline_IV)";
        descriptionShort = "CSTRING(Saline_IV_Desc_Short)";
    };
    
    class ACE_salineIV_500: ACE_salineIV {
        displayName = "CSTRING(Saline_IV_500)";
    };
    
    class ACE_salineIV_250: ACE_salineIV {
        displayName = "CSTRING(Saline_IV_250)";
    };
    
    class ACE_bodyBag: ACE_ItemCore {
        scope = 2;
        ACE_isMedicalItem = 1;
        displayName = "$STR_a3_cfgvehicles_land_bodybag_01_black_f0";
        descriptionShort = "CSTRING(Bodybag_Desc_Short)";
    };
    
    class ACE_bodyBag_blue: ACE_bodyBag {
        displayName = "$STR_a3_cfgvehicles_land_bodybag_01_blue_f0";
    };
    
    class ACE_bodyBag_white: ACE_bodyBag {
        displayName = "$STR_a3_cfgvehicles_land_bodybag_01_white_f0";
    };
}; 