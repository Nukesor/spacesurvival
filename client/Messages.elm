module Messages exposing (..)

import Animation
import Http
import Json.Decode
import Model exposing (MainView)
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
    | ReadLocalToken LoggedInData
    | ShowBuildDialog (Maybe Point)
    | ReceiveResearches (Result Http.Error Model.Research.Researches)
    | SetMainView MainView
    | ReceiveQueue (Result Http.Error Queue)
    | QueueEntryAdded (Result Http.Error Json.Decode.Value)
    | StartResearching String
    | ReceiveAvailableModules (Result Http.Error AvailableModules)
    | ReceiveResources (Result Http.Error Resources)
    | ReceiveGrid (Result Http.Error Grid)
    | Tick Time
    | StartBuilding ModuleId Point
