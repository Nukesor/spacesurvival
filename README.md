# Spacesurvival
An incremental, collaborative survival game set in a futuristic deep space scenario.

## requirements
- python 3 / `virtualenv`
- npm
- postgresql

## running
If running for the first time:

    bin/setup
    createdb browsergame-dev
    createdb browsergame-test


Then, to start the server, watch and rebuild the frontend, etc.:

    ./watch

## tests

    source venv/bin/activate
    pytest
