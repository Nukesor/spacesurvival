module Model.Queue exposing (..)

import Model.Research exposing (ResearchId)
import Time.Date exposing (Date)


type alias ResearchData =
    { createdAt : Date
    , id : String
    , researchId : ResearchId
    , level : Int
    }


type alias ModuleData =
    { createdAt : Date
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
