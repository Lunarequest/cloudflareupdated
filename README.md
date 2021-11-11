# CloudFlareupdated

![](https://img.shields.io/github/workflow/status/Lunarequest/cloudflareupdated/Build?label=Checks&style=for-the-badge)

If you self host often times one of the major issues you will run into is needing to update your Dynamic ip address. There are many solutions to deal with this issue, in fact I even wrote a python script for this.

The python script ran into many issues and was close to unmaintaibale, This version plans to reduce the complexity involved with error checking by using rust. Some of the benefits include being faster then the python version and being able to rapidly check and update domains and ip addresses. 

All of the code in this project is async and hence lends to faster speeds and all round better performance. This project is intended to be used as a demon, you can see examples for a systemd timer in [configs](./configs)

To configure the application a settings file is required. An example one is give in [configs](./configs/example-settings.yaml). By default cloudflareupdated assumes the config file is at `./settings.yaml` to change this use the `-c`/`--config` arguments.

Optionally there is experimental support for sending emails when a domain is updated. 
