module Main exposing (..)

import Html
import View exposing (..)
import Model exposing (..)
import Model.User exposing (..)
import Messages exposing (..)
import Animation
import Array
import Model.Grid exposing (Point, Grid, Slot)
import Animations
import Api.Auth


main : Program Never Model Msg
main =
    Html.program { init = init, update = update, subscriptions = subscriptions, view = View.view }


init : ( Model, Cmd msg )
init =
    { grid = createGrid, user = LoggingIn { identifier = "", password = "" }, authDialogAnimation = Animation.interrupt Animations.dialogAppear Animations.dialogAppearStyle, authView = Model.Login } ! []


createGrid : Grid
createGrid =
    Array.initialize 10 (\x -> Array.initialize 10 (\y -> { position = Point x y }))


subscriptions : Model -> Sub Msg
subscriptions model =
    Animation.subscription AnimateModal [ model.authDialogAnimation ]


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        AnimateModal frame ->
            { model | authDialogAnimation = Animation.update frame model.authDialogAnimation } ! []

        Messages.Login ->
            model ! [ Api.Auth.login model ]

        Messages.Register ->
            model ! [ Api.Auth.register model ]

        ChangeAuthView view ->
            { model | authView = view } ! []

        UpdateUser user ->
            { model | user = user } ! []

        Registered result ->
            case Debug.log "register result" result of
                Ok user ->
                    { model | authView = Model.Login, user = Model.User.LoggingIn user } ! []

                Err _ ->
                    model ! []

        Messages.LoggedIn result ->
            case Debug.log "login result" result of
                Ok user ->
                    { model | user = Model.User.LoggedIn user } ! []

                Err err ->
                    model ! []
