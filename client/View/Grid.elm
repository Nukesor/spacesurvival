module View.Grid exposing (..)

import Svg exposing (..)
import Svg.Attributes exposing (..)
import Svg.Events exposing (..)
import Model.Grid as Grid exposing (Slot, Grid)
import Model.Util exposing (..)
import Model exposing (..)
import Html exposing (..)
import Messages exposing (..)
import View.BuildDialog


view : Model -> Html.Html Msg
view model =
    div []
        [ svg []
            (Grid.map
                slot
                model.grid
            )
        , View.BuildDialog.view model
        ]


slot : Slot -> Svg Msg
slot slot =
    let
        ( xp, yp ) =
            toPercentage slot.position
    in
        image
            [ xlinkHref "/static/img/grid_slot.svg"
            , x xp
            , y yp
            , width "10%"
            , height "10%"
            , class "slot"
            , onClick (ShowBuildDialog (Just slot.position))
            ]
            []


toPercentage : Point -> ( String, String )
toPercentage point =
    ( (toString <| point.x * 10) ++ "%", (toString <| point.y * 10) ++ "%" )
