module Model.Queue exposing (..)

import Model.Research exposing (ResearchId)
import Time.DateTime exposing (DateTime)


type alias ResearchData =
    { createdAt : DateTime
    , id : String
    , researchId : ResearchId
    , level : Int
    }


type alias ModuleData =
    { createdAt : DateTime
    , id : String
    , moduleId : ResearchId
    , name : String
    , level : Int
    }


type Entry
    = ResearchEntry ResearchData
    | ModuleEntry ModuleData


type alias Queue =
    List Entry
