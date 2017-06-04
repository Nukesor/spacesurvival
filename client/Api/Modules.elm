module Api.Modules exposing (..)

import Api.Util exposing (authenticatedGet, dataDecoder, pairDecoder)
import Array
import Dict
import Json.Decode as Decode
import Json.Decode.Extra exposing ((|:))
import Messages
import Model exposing (Model)
import Model.Grid exposing (Grid, setAtPosition)
import Model.Modules exposing (Module, ModuleLevel, ModuleType, Shoots)


modulesDecoder : Decode.Decoder (Dict.Dict String ModuleType)
modulesDecoder =
    dataDecoder <|
        Decode.dict moduleDecoder


gridDecoder : Decode.Decoder Grid
gridDecoder =
    dataDecoder <|
        Decode.map slotsToGrid (Decode.list gridSlotDecoder)


slotsToGrid : List GridSlot -> Grid
slotsToGrid =
    List.foldl
        (\slot grid ->
            setAtPosition slot.x slot.y (Module slot.id slot.level) grid
        )
        Model.Grid.empty


type alias GridSlot =
    { level : Int
    , id : String
    , x : Int
    , y : Int
    }


gridSlotDecoder =
    Decode.succeed GridSlot
        |: (Decode.field "level" Decode.int)
        |: (Decode.field "name" Decode.string)
        |: (Decode.field "x_pos" Decode.int)
        |: (Decode.field "y_pos" Decode.int)


moduleDecoder : Decode.Decoder ModuleType
moduleDecoder =
    Decode.succeed ModuleType
        |: (Decode.field "name" Decode.string)
        |: (Decode.field "dependencies" (Decode.list (pairDecoder Decode.string Decode.int)))
        |: (Decode.field "levels" (Decode.list moduleLevelDecoder))


moduleLevelDecoder : Decode.Decoder ModuleLevel
moduleLevelDecoder =
    Decode.succeed ModuleLevel
        |: (Decode.field "level" Decode.int)
        |: (Decode.field "consumes" (Decode.list resourceAmountDecoder))
        |: (Decode.field "generates" (Decode.list resourceAmountDecoder))
        |: (Decode.field "resources" (Decode.list resourceAmountDecoder))
        |: (Decode.field "shoots" (Decode.maybe shootsDecoder))
        |: (Decode.field "time" Decode.int)


shootsDecoder : Decode.Decoder Shoots
shootsDecoder =
    Decode.succeed Shoots
        |: (Decode.field "damage" Decode.int)
        |: (Decode.field "range" Decode.int)
        |: (Decode.field "rate" Decode.int)


resourceAmountDecoder : Decode.Decoder ( String, Int )
resourceAmountDecoder =
    pairDecoder Decode.string Decode.int


fetchAvailableModules : Model -> Cmd Messages.Msg
fetchAvailableModules model =
    authenticatedGet model "/api/modules" modulesDecoder Messages.ReceiveAvailableModules


fetchGridModules : Model -> Cmd Messages.Msg
fetchGridModules model =
    authenticatedGet model "/api/modules/pod" gridDecoder Messages.ReceiveGrid
