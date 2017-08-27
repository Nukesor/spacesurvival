module Model.Modules exposing (..)

import Dict
import Model.Research exposing (ResearchId, ResourceId, dependencyFulfilled)
import Time


type alias SlotEntry =
    Maybe Module


type alias ModuleId =
    String


type alias AvailableModules =
    Dict.Dict ModuleId ModuleType


type alias Module =
    { id : ModuleId, level : Int }


type alias ModuleType =
    { name : String
    , dependencies : List ( ResearchId, Int )
    , levels : List ModuleLevel
    }


type alias ModuleLevel =
    { level : Int
    , consumes : List ( ResourceId, Int )
    , generates : List ( ResourceId, Int )
    , buildCosts : List ( ResourceId, Int )
    , shoots :
        Maybe Shoots
    , buildTime : Int
    }


type alias Shoots =
    { damage : Int
    , range : Int
    , rate : Int
    }


type alias UserModules =
    {}


buildableModules : Model.Research.Researches -> AvailableModules -> AvailableModules
buildableModules researches modules =
    Dict.filter (\key mod -> isBuildable researches mod) modules


isBuildable : Model.Research.Researches -> ModuleType -> Bool
isBuildable researches mod =
    List.all (dependencyFulfilled researches) mod.dependencies



-- tick : Time.Time -> List Module -> AvailableModules -> Resources -> Resources
-- tick dt modules moduleSpecs resources =
--     modules
--         |> List.filterMap (\mod -> findCurrentLevel moduleSpecs mod)
--         |> List.map (\level -> )


findCurrentLevel : AvailableModules -> Module -> Maybe ModuleLevel
findCurrentLevel specs mod =
    specs
        |> Dict.get mod.id
        |> Maybe.map (\spec -> List.filter (\level -> level.level == mod.level) spec.levels)
        |> Maybe.andThen List.head


resourcesPerTick : Time.Time -> List ( ResourceId, Int ) -> List ( ResourceId, Int )
resourcesPerTick dt =
    List.map
        (\( id, hourlyProduction ) ->
            ( id, applyDeltaTime dt hourlyProduction )
        )


applyDeltaTime : Time.Time -> Int -> Int
applyDeltaTime time hourlyValue =
    let
        perSecond =
            (toFloat hourlyValue) / (60 * 60)
    in
        round (perSecond / (Time.inSeconds time))
