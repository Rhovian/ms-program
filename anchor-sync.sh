#!/bin/bash

set -ex

rm -rf target/deploy/
anchor build
anchor keys sync
anchor build
anchor deploy 
