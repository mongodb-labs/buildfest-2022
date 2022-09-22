<?php namespace SchoolApp\Repository;

use MongoDB\Driver\Monitoring\CommandFailedEvent;
use MongoDB\Driver\Monitoring\CommandStartedEvent;
use MongoDB\Driver\Monitoring\CommandSucceededEvent;

class EventSubscriber implements \MongoDB\Driver\Monitoring\CommandSubscriber
{
    public function commandSucceeded(CommandSucceededEvent $event): void
    {
        $this->writeToLog("Command Succeeded!\n");
    }
    public function commandFailed(CommandFailedEvent $event): void
    {
        $this->writeToLog("Command Failed!\n");
    }
    public function commandStarted(CommandStartedEvent $event): void
    {
        $this->writeToLog("Command Started!\n");
    }
    private function writeToLog(string $msg){
        $myfile = fopen("/school_app/logfile.txt", "a");
        fwrite($myfile, $msg);
        fclose($myfile);
    }
}