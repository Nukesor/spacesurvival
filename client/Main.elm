module Main exposing (..)

import Animation
import Animations
import Api.Auth
import Dict
import Html
import Messages exposing (..)
import Model exposing (..)
import Model.Grid exposing (Grid, Slot)
import Model.User exposing (..)
import Task exposing (perform)
import Time exposing (every, second)
import Time.DateTime exposing (DateTime, dateTime)
import Update exposing (..)
import View exposing (..)


main : Program Flags Model Msg
main =
    Html.programWithFlags { init = init, update = update, subscriptions = subscriptions, view = View.view }


type alias Flags =
    { timeZoneOffset : Float }


init : Flags -> ( Model, Cmd Msg )
init flags =
    { pod = Model.Grid.empty
    , base = Model.Grid.empty
    , user = LoggingIn { identifier = "", password = "" }
    , authDialogAnimation = Animation.interrupt Animations.dialogAppear Animations.dialogAppearStyle
    , mainView = GridView Pod
    , availableModules =
        Dict.empty
    , researches = Dict.empty
    , buildingAt = Nothing
    , queue = []
    , resources = []
    , currentDate = dateTime Time.DateTime.zero
    , lastTick = Nothing
    , timeZoneOffset = flags.timeZoneOffset
    }
        ! [ Api.Auth.readToken ()
          , perform Tick Time.now
          ]


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.batch
        [ Animation.subscription AnimateModal [ model.authDialogAnimation ]
        , Api.Auth.receiveToken
            (\user -> Messages.LoggedIn (Ok user))
        , every second Tick
        ]
