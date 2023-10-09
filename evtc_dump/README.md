# evtc_dump

Dump ArcDPS EVTC log information in a JSON format.

```sh
# dump all information
evtc_dump path/to/log.zevtc
evtc_dump path/to/log.zevtc path/to/output.json

# dump specific information
evtc_dump path/to/log.zevtc --data agents
evtc_dump path/to/log.zevtc --data skills
evtc_dump path/to/log.zevtc --data events
```
