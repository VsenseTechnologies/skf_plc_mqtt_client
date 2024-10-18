#!/bin/bash

temp=0
pid=0
counter=0
trip=0
run=1

temp1=0
temp2=0
temp3=0
temp4=0

time1=60
time2=60
time3=60
time4=60

step=1

timer=0

while true; do

        real_time_temp="{\"RegAd\":\"302\",\"D1\":\"$temp\"}"
        real_time_pid="{\"RegAd\":\"322\",\"D1\":\"$pid\"}"
        blower_trip_st="{\"RegAd\":\"18\",\"D1\":\"$trip\"}"
        elevator_trip_st="{\"RegAd\":\"20\",\"D1\":\"$trip\"}"
        rotor_trip_st="{\"RegAd\":\"22\",\"D1\":\"$trip\"}"
        blower_run_st="{\"RegAd\":\"24\",\"D1\":\"$run\"}"
        elevator_run_st="{\"RegAd\":\"26\",\"D1\":\"$run\"}"
        rotor_run_st="{\"RegAd\":\"28\",\"D1\":\"$run\"}"
        st1_temp="{\"RegAd\":\"304\",\"D1\":\"$temp1\"}"
        st2_temp="{\"RegAd\":\"306\",\"D1\":\"$temp2\"}"
        st3_temp="{\"RegAd\":\"308\",\"D1\":\"$temp3\"}"
        st4_temp="{\"RegAd\":\"310\",\"D1\":\"$temp4\"}"
        st1_time="{\"RegAd\":\"124\",\"D1\":\"$time1\"}"
        st2_time="{\"RegAd\":\"126\",\"D1\":\"$time2\"}"
        st3_time="{\"RegAd\":\"128\",\"D1\":\"$time3\"}"
        st4_time="{\"RegAd\":\"130\",\"D1\":\"$time4\"}"

        mosquitto_pub -h skfplc.vsensetech.in -p 1883 -t vs24skf01/skf/data -m "$real_time_temp"
        mosquitto_pub -h skfplc.vsensetech.in -p 1883 -t vs24skf01/skf/data -m "$real_time_pid"
        mosquitto_pub -h skfplc.vsensetech.in -p 1883 -t vs24skf01/skf/data -m "$blower_trip_st"
        mosquitto_pub -h skfplc.vsensetech.in -p 1883 -t vs24skf01/skf/data -m "$elevator_trip_st"
        mosquitto_pub -h skfplc.vsensetech.in -p 1883 -t vs24skf01/skf/data -m "$rotor_trip_st"
        mosquitto_pub -h skfplc.vsensetech.in -p 1883 -t vs24skf01/skf/data -m "$blower_run_st"
        mosquitto_pub -h skfplc.vsensetech.in -p 1883 -t vs24skf01/skf/data -m "$elevator_run_st"
        mosquitto_pub -h skfplc.vsensetech.in -p 1883 -t vs24skf01/skf/data -m "$rotor_run_st"
        mosquitto_pub -h skfplc.vsensetech.in -p 1883 -t vs24skf01/skf/data -m "$st1_temp"
        mosquitto_pub -h skfplc.vsensetech.in -p 1883 -t vs24skf01/skf/data -m "$st2_temp"
        mosquitto_pub -h skfplc.vsensetech.in -p 1883 -t vs24skf01/skf/data -m "$st3_temp"
        mosquitto_pub -h skfplc.vsensetech.in -p 1883 -t vs24skf01/skf/data -m "$st4_temp"
        mosquitto_pub -h skfplc.vsensetech.in -p 1883 -t vs24skf01/skf/data -m "$st1_time"
        mosquitto_pub -h skfplc.vsensetech.in -p 1883 -t vs24skf01/skf/data -m "$st2_time"
        mosquitto_pub -h skfplc.vsensetech.in -p 1883 -t vs24skf01/skf/data -m "$st3_time"
        mosquitto_pub -h skfplc.vsensetech.in -p 1883 -t vs24skf01/skf/data -m "$st4_time"

        # Reset temp if it's greater than 200
        if [ $temp -gt 200 ]; then
            temp=0
        fi

        # Reset pid if it's greater than 20
        if [ $pid -gt 20 ]; then
            pid=0
        fi

        # Toggle trip and run every time counter exceeds 5
        if [ $counter -gt 5 ]; then
            ((trip = !trip))   # Toggle trip
            ((run = !run))     # Toggle run
            counter=0          # Reset counter
        fi

        # Reset timer every 60 seconds and increment step
        if [ $timer -gt 60 ]; then
            timer=0
            ((step += 1))
        fi

        # Reset step and time/temp values when step reaches 5
        if [ $step -eq 5 ]; then
            step=1
            time1=60
            time2=60
            time3=60
            time4=60
            temp1=0
            temp2=0
            temp3=0
            temp4=0
        fi

        # Adjust time and temperature based on the current step
        if [ $step -eq 1 ]; then
            ((time1 = time1 - 1))
            ((temp1 = temp1 + 3))
        elif [ $step -eq 2 ]; then
            ((time2 = time2 - 1))
            ((temp2 = temp2 + 3))
        elif [ $step -eq 3 ]; then
            ((time3 = time3 - 1))
            ((temp3 = temp3 + 3))
        else
            ((time4 = time4 - 1))
            ((temp4 = temp4 + 3))
        fi

        # Increment variables for each loop iteration
        ((temp += 10))
        ((pid += 1))
        ((counter += 1))
        ((timer += 1))

        # Sleep for 1 second before next iteration
        sleep 1
done
