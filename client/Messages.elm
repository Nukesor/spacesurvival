module Messages exposing (..)

import Animation
import Http
import Json.Decode
import Model exposing (MainView, SelectedGrid)
import Model.Grid exposing (Grid)
import Model.Modules exposing (AvailableModules, ModuleId)
import Model.Queue exposing (Queue)
import Model.Research
import Model.Resources exposing (Resources)
import Model.User exposing (..)
import Model.Util exposing (..)
import Time exposing (Time)


type Msg
    = AnimateModal Animation.Msg
    | Login
    | Register
    | UpdateUser User
    | Registered (Result Http.Error LoginData)
    | LoggedIn (Result Http.Error LoggedInData)
    | ShowBuildDialog (Maybe Point)
    | ReceiveAvailableResearches (Result Http.Error Model.Research.Researches)
    | ReceiveResearchLevels (Result Http.Error (List ( Model.Research.ResearchId, Int )))
    | SetMainView MainView
    | ReceiveQueue (Result Http.Error Queue)
    | StartResearching String
    | ReceiveAvailableModules (Result Http.Error AvailableModules)
    | ReceiveResources (Result Http.Error Resources)
    | ReceiveGrid (Result Http.Error Grid)
    | Tick Time
    | StartBuilding ModuleId Point
    | Upgrade ModuleId
    | Noop (Result Http.Error Json.Decode.Value)
    | Command (Cmd Msg)


{-| Useful for executing another command after completing an API call.
-}
commandAsMsg : Cmd Msg -> a -> Msg
commandAsMsg cmd _ =
    Command cmd
