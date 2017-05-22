module Update exposing (..)

import Messages exposing (..)
import Api.Auth
import Animation
import Model exposing (..)
import Model.User


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
                    { model | user = Model.User.LoggedIn user }
                        ! [ Api.Auth.saveToken user.token
                          ]

                Err err ->
                    model ! []

        Messages.ReadLocalToken user ->
            { model | user = Model.User.LoggedIn user } ! []

        ShowBuildDialog maybePoint ->
            { model
                | buildingAt = Debug.log "point" maybePoint
            }
                ! []
