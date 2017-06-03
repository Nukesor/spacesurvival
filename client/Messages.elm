module Messages exposing (..)

import Animation
import Http
import Model exposing (MainView)
import Model.Modules exposing (Modules)
import Model.Queue exposing (Queue)
import Model.Research
import Model.User exposing (..)
import Model.Util exposing (..)


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
    | ReceiveQueueEntry (Result Http.Error Model.Queue.Entry)
    | StartResearching String
    | ReceiveAvailableModules (Result Http.Error Modules)
