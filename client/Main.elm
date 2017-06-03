module Main exposing (..)

import Animation
import Animations
import Api.Auth
import Array
import Dict
import Html
import Messages exposing (..)
import Model exposing (..)
import Model.Grid exposing (Grid, Slot)
import Model.User exposing (..)
import Model.Util exposing (..)
import Task exposing (perform)
import Time exposing (every, second)
import Time.DateTime exposing (DateTime, dateTime)
import Update exposing (..)
import View exposing (..)


main : Program Never Model Msg
main =
    Html.program { init = init, update = update, subscriptions = subscriptions, view = View.view }


init : ( Model, Cmd Msg )
init =
    { grid = createGrid
    , user = LoggingIn { identifier = "", password = "" }
    , authDialogAnimation = Animation.interrupt Animations.dialogAppear Animations.dialogAppearStyle
    , mainView = GridView
    , availableModules =
        Dict.empty
    , researches = Dict.empty
    , buildingAt = Nothing
    , queue = []
    , resources = []
    , currentDate = dateTime Time.DateTime.zero
    }
        ! [ Api.Auth.readToken ()
          , perform Tick Time.now
          ]


createGrid : Grid
createGrid =
    Array.initialize 10
        (\x ->
            Array.initialize 10
                (\y ->
                    { position = Point x y
                    , entry = Nothing
                    }
                )
        )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.batch
        [ Animation.subscription AnimateModal [ model.authDialogAnimation ]
        , Api.Auth.receiveToken
            (\token -> Messages.LoggedIn (Ok { token = token }))
        , every second Tick
        ]
