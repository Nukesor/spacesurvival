module Api.Modules exposing (..)

import Api.Util exposing (authenticatedGet, authenticatedPost, pairDecoder, podUrl)
import Dict
import Json.Decode as Decode
import Json.Decode.Extra exposing ((|:))
import Json.Encode as Encode
import Messages exposing (Msg(QueueEntryAdded))
import Model exposing (Model)
import Model.Grid exposing (Grid, setAtPosition)
import Model.Modules exposing (Module, ModuleId, ModuleLevel, ModuleType, Shoots)
import Model.Util exposing (Point)


modulesDecoder : Decode.Decoder (Dict.Dict String ModuleType)
modulesDecoder =
    Decode.dict moduleTypeDecoder


gridDecoder : Decode.Decoder Grid
gridDecoder =
    Decode.map slotsToGrid (Decode.list gridSlotDecoder)


slotsToGrid : List GridSlot -> Grid
slotsToGrid =
    List.foldl
        (\slot grid ->
            setAtPosition slot.point (Module slot.id slot.level slot.uuid) grid
        )
        Model.Grid.empty


type alias GridSlot =
    { level : Int
    , id : String
    , uuid : String
    , point : Point
    }


toGridSlot : Int -> String -> String -> Int -> Int -> GridSlot
toGridSlot level id uuid x y =
    { level = level, id = id, point = Point x y, uuid = uuid }


gridSlotDecoder : Decode.Decoder GridSlot
gridSlotDecoder =
    Decode.succeed toGridSlot
        |: (Decode.field "level" Decode.int)
        |: (Decode.field "name" Decode.string)
        |: (Decode.field "id" Decode.string)
        |: (Decode.field "x_pos" Decode.int)
        |: (Decode.field "y_pos" Decode.int)


moduleTypeDecoder : Decode.Decoder ModuleType
moduleTypeDecoder =
    Decode.succeed ModuleType
        |: (Decode.field "display_name" Decode.string)
        |: (Decode.field "dependencies" (Decode.list dependencyDecoder))
        |: (Decode.field "levels" (Decode.list moduleLevelDecoder))


dependencyDecoder =
    Decode.map2 (,)
        (Decode.field "type" Decode.string)
        (Decode.field "level" Decode.int)


moduleLevelDecoder : Decode.Decoder ModuleLevel
moduleLevelDecoder =
    Decode.succeed ModuleLevel
        |: (Decode.field "level" Decode.int)
        |: (Decode.field "consumes" (Decode.list resourceAmountDecoder))
        |: (Decode.field "generates" (Decode.list resourceAmountDecoder))
        |: (Decode.field "resources" (Decode.list resourceAmountDecoder))
        |: (Decode.maybe (Decode.field "shoots" shootsDecoder))
        |: (Decode.field "duration" Decode.int)


shootsDecoder : Decode.Decoder Shoots
shootsDecoder =
    Decode.succeed Shoots
        |: (Decode.field "damage" Decode.int)
        |: (Decode.field "range" Decode.int)
        |: (Decode.field "rate" Decode.int)


newModuleEncoder : String -> Point -> Encode.Value
newModuleEncoder mod_type point =
    Encode.object
        [ ( "module_type", Encode.string mod_type )
        , ( "stationary", Encode.bool False )
        , ( "position_x", Encode.int point.x )
        , ( "position_y", Encode.int point.y )
        ]


resourceAmountDecoder : Decode.Decoder ( String, Int )
resourceAmountDecoder =
    Decode.map2 (,)
        (Decode.field "type" Decode.string)
        (Decode.field "amount" Decode.int)


fetchAvailableModules : Model -> Cmd Messages.Msg
fetchAvailableModules model =
    authenticatedGet model "/api/modules" modulesDecoder Messages.ReceiveAvailableModules


fetchGridModules : Model -> Cmd Messages.Msg
fetchGridModules model =
    authenticatedGet model (podUrl model.user "/modules") gridDecoder Messages.ReceiveGrid


startBuilding : Model -> ModuleId -> Point -> Cmd Messages.Msg
startBuilding model id point =
    authenticatedPost model
        "/api/modules/pod/new"
        Decode.value
        QueueEntryAdded
        (newModuleEncoder id point)


upgrade : Model -> String -> Cmd Messages.Msg
upgrade model uuid =
    authenticatedPost model
        ("/api/modules/pod/upgrade/" ++ uuid)
        Decode.value
        QueueEntryAdded
        Encode.null
