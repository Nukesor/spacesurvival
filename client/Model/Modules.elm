module Model.Modules exposing (..)


type alias SlotEntry =
    Maybe Module


type alias ModuleId =
    String


type alias Module =
    { name : String
    , id : ModuleId
    }


type alias UserModules =
    {}
