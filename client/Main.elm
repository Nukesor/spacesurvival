module Main exposing (..)

import Html
import Api.Auth
import View exposing (..)
import Model exposing (..)
import Model.User exposing (..)
import Messages exposing (..)
import Animation
import Array
import Model.Grid exposing (Grid, Slot)
import Animations
import Model.Util exposing (..)
import Update exposing (..)
import Dict


main : Program Never Model Msg
main =
    Html.program { init = init, update = update, subscriptions = subscriptions, view = View.view }


init : ( Model, Cmd msg )
init =
    { grid = createGrid
    , user = LoggingIn { identifier = "", password = "" }
    , authDialogAnimation = Animation.interrupt Animations.dialogAppear Animations.dialogAppearStyle
    , authView = Model.Login
    , availableModules =
        [ { name = "hypercharge capacitor"
          , id = "1"
          }
        , { name = "flux capacitor"
          , id = "2"
          }
        ]
    , researches = Dict.empty
    }
        ! [ Api.Auth.readToken ()
          ]


createGrid : Grid
createGrid =
    Array.initialize 10
        (\x ->
            Array.initialize 10
                (\y ->
                    { position = Point x y
                    , entry = Nothing
                    , selectedForBuilding = False
                    }
                )
        )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.batch
        [ Animation.subscription AnimateModal [ model.authDialogAnimation ]
        , Api.Auth.receiveToken (\token -> Messages.LoggedIn (Ok { token = token }))
        ]
