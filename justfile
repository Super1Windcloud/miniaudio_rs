default:
    echo 'Hello, world!'


capture:
   gcc c_examples/simple_capture.c -o simple_capture && ./simple_capture  output.wav


playback:
   gcc c_examples/simple_playback.c -o simple_playback && ./simple_playback  output.wav

looping:
   gcc c_examples/simple_looping.c -o simple_looping && ./simple_looping  output.wav

loopback:
   gcc c_examples/simple_loopback.c -o simple_loopback && ./simple_loopback  output.wav

push:
    git pull  repo  main:master
    git add  .
    git commit -m "update"
    git push repo  master:main