module Model.Modules exposing (..)

import Dict
import List.Extra
import Model.Research exposing (ResearchId, ResourceId, dependencyFulfilled)
import Model.Resources exposing (Resources, applyTick, foldModifications, resourcesExist)
import Time


type alias SlotEntry =
    Maybe Module


type alias ModuleId =
    String


type alias Uuid =
    String


type alias AvailableModules =
    Dict.Dict ModuleId ModuleType


type alias Module =
    { id : ModuleId, level : Int, uuid : Uuid }


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


tick : Time.Time -> List Module -> AvailableModules -> Resources -> Resources
tick dt modules moduleSpecs resources =
    let
        canExecute level =
            resourcesExist resources level.consumes

        levels =
            modules
                |> List.filterMap (\mod -> findCurrentLevel moduleSpecs mod)
                |> List.filter canExecute

        toModifications modificationsPerModule =
            modificationsPerModule
                |> List.map (resourcesPerTick dt)
                |> foldModifications

        input =
            levels
                |> List.map .consumes
                |> toModifications

        output =
            levels
                |> List.map .generates
                |> toModifications
    in
        resources
            |> applyTick (-) input
            |> applyTick (+) output


findCurrentLevel : AvailableModules -> Module -> Maybe ModuleLevel
findCurrentLevel specs mod =
    findLevel specs mod mod.level


findLevel : AvailableModules -> Module -> Int -> Maybe ModuleLevel
findLevel specs mod level =
    specs
        |> Dict.get mod.id
        |> Maybe.map .levels
        |> Maybe.andThen (List.Extra.find (.level >> (==) level))


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
        round (perSecond * (Time.inSeconds time))
