#!/usr/bin/env php
<?php
declare(ticks=1);

if (php_sapi_name() !== 'cli') {
    echo 'Must be used from CLI';
    die;
}

// signal handler function
function sig_handler($signo)
{
    switch ($signo) {
        case SIGTERM:
            syslog(LOG_DEBUG, 'SIGTERM');
            // handle shutdown tasks
            exit;
            break;
        case SIGINT:
            syslog(LOG_DEBUG, 'SIGINT');
            break;
        case SIGHUP:
            syslog(LOG_DEBUG, 'SIGHUP');
            // handle restart tasks
            break;
        case SIGUSR1:
            syslog(LOG_DEBUG, 'SIGUSR1');
            break;
        case SIGKILL:
            syslog(LOG_DEBUG, 'SIGKILL');
            break;
        default:
            syslog(LOG_DEBUG, $signo);
        // handle all other signals
    }
}

echo "Installing signal handler...\n";

// setup signal handlers
pcntl_signal(SIGCHLD, "sig_handler");
//pcntl_signal(SIGINT, "sig_handler");
pcntl_signal(SIGQUIT, "sig_handler");
pcntl_signal(SIGTERM, "sig_handler");
pcntl_signal(SIGTERM, "sig_handler");
pcntl_signal(SIGHUP, "sig_handler");
pcntl_signal(SIGUSR1, "sig_handler");
//pcntl_signal(SIGKILL, "sig_handler");

while (true) {
    syslog(LOG_DEBUG, 'Tick');
    echo 'Tick ' . time() . PHP_EOL;
    sleep(2);
}
