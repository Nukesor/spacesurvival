module Update exposing (..)

import Animation
import Api.Auth
import Api.Research
import Messages exposing (..)
import Model exposing (..)
import Model.User exposing (LoginData)


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
                    let
                        updatedModel =
                            { model | user = Model.User.LoggedIn user }
                    in
                        updatedModel
                            ! [ Api.Auth.saveToken user.token
                              , Api.Research.fetchResearches updatedModel
                              ]

                Err err ->
                    model ! []

        Messages.ReadLocalToken user ->
            { model | user = Model.User.LoggedIn user } ! []

        ShowBuildDialog maybePoint ->
            { model
                | buildingAt = maybePoint
            }
                ! []

        ReceiveResearches result ->
            case Debug.log "researches" result of
                Ok researches ->
                    { model | researches = researches } ! []

                Err err ->
                    { model | user = Model.User.LoggingIn { identifier = "", password = "" } } ! []
