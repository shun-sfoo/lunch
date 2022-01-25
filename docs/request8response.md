# title

## request

first

```xml

<?xml version="1.0" encoding="UTF-8"?>
<request requestid="7b93cac8-3e13-42ab-b670-c61c5e92f367" sessionid="6562ebef-dce5-46d5-ab7b-c9add97ca20d" protocol="3.0" updater="ChromeOSUpdateEngine" updaterversion="0.1.0.0" installsource="ondemandupdate" ismachine="1">
    <os version="Indy" platform="Chrome OS" sp="14092.106.2021_11_08_1354_x86_64" market_segment="consumer"></os>
    <app appid="{87efface-864d-49a5-9bb3-4b050a7c227a}" version="14092.106.2021_11_08_1354" track="stable-channel" board="amd64-generic" hardware_class="" delta_okay="true" installdate="5418" >
        <ping active="1" a="3" r="3"></ping>
        <updatecheck></updatecheck>
    </app>
</request>


<?xml version="1.0" encoding="UTF-8"?>
<request requestid="75b1fbc9-16cf-4570-858b-b3fd019a225f" sessionid="307cfc33-0226-41b3-8183-789857a62b10" protocol="3.0" updater="ChromeOSUpdateEngine" updaterversion="0.1.0.0" installsource="ondemandupdate" ismachine="1">
    <os version="Indy" platform="Chrome OS" sp="14092.106.2021_11_08_1016_x86_64" market_segment="consumer"></os>
    <app appid="{87efface-864d-49a5-9bb3-4b050a7c227a}" version="14092.106.2021_11_08_1016" track="stable-channel" board="amd64-generic" hardware_class="" delta_okay="true" installdate="5418" >
        <ping active="1" a="-1" r="-1"></ping>
        <updatecheck></updatecheck>
    </app>
</request>
```

second

```xml
<?xml version="1.0" encoding="UTF-8"?>
<request requestid="6945983e-2799-45f2-a79e-7cb492c0dcac" sessionid="307cfc33-0226-41b3-8183-789857a62b10" protocol="3.0" updater="ChromeOSUpdateEngine" updaterversion="0.1.0.0" installsource="ondemandupdate" ismachine="1">
    <os version="Indy" platform="Chrome OS" sp="14092.106.2021_11_08_1016_x86_64" market_segment="consumer"></os>
    <app appid="{87efface-864d-49a5-9bb3-4b050a7c227a}" version="14092.106.2021_11_08_1016" track="stable-channel" board="amd64-generic" hardware_class="" delta_okay="true" installdate="5418" >
        <event eventtype="13" eventresult="1"></event>
    </app>
</request>
```

third

```xml
<?xml version="1.0" encoding="UTF-8"?>
<request requestid="6032501a-1136-4c9a-b9b7-b2e1ec4cca68" sessionid="307cfc33-0226-41b3-8183-789857a62b10" protocol="3.0" updater="ChromeOSUpdateEngine" updaterversion="0.1.0.0" installsource="ondemandupdate" ismachine="1">
    <os version="Indy" platform="Chrome OS" sp="14092.106.2021_11_08_1016_x86_64" market_segment="consumer"></os>
    <app appid="{87efface-864d-49a5-9bb3-4b050a7c227a}" version="14092.106.2021_11_08_1016" track="stable-channel" board="amd64-generic" hardware_class="" delta_okay="true" installdate="5418" >
        <event eventtype="14" eventresult="1"></event>
    </app>
</request>

```

fouth

```xml
<?xml version="1.0" encoding="UTF-8"?>
<request requestid="2994ef91-5b21-4b79-8c44-124ebae9980d" sessionid="307cfc33-0226-41b3-8183-789857a62b10" protocol="3.0" updater="ChromeOSUpdateEngine" updaterversion="0.1.0.0" installsource="ondemandupdate" ismachine="1">
    <os version="Indy" platform="Chrome OS" sp="14092.106.2021_11_08_1016_x86_64" market_segment="consumer"></os>
    <app appid="{87efface-864d-49a5-9bb3-4b050a7c227a}" version="14092.106.2021_11_08_1016" track="stable-channel" board="amd64-generic" hardware_class="" delta_okay="true" installdate="5418" >
        <event eventtype="3" eventresult="1"></event>
    </app>
</request>

```

## response

first

```xml
<?xml version="1.0" encoding="UTF-8"?>
<response protocol="3.0" server="nebraska">
  <daystart elapsed_days="5425" elapsed_seconds="54482"/>
  <app appid="{87efface-863d-49a5-9bb3-4b050a7c227a}" status="ok">
    <ping status="ok"/>
    <updatecheck status="ok">
      <urls>
        <url codebase="http://192.168.31.120:8080/"/>
      </urls>
      <manifest version="99999.0.0">
        <actions>
          <action event="update" run="delta.bin"/>
          <action ChromeOSVersion="99999.0.0" ChromeVersion="1.0.0.0" DisablePayloadBackoff="false" IsDeltaPayload="false" MaxDaysToScatter="14" MetadataSignatureRsa="" MetadataSize="57970" sha256="NbXXg6SyQdXiD+j1/RMqgAfA+sIgFUUPBl6t2+dUgxw=" event="postinstall"/>
        </actions>
        <packages>
          <package fp="1.35B5D783A4B241D5E20FE8F5FD132A8007C0FAC22015450F065EADDBE754831C" hash_sha256="35B5D783A4B241D5E20FE8F5FD132A8007C0FAC22015450F065EADDBE754831C" name="delta.bin" required="true" size="653474911"/>
        </packages>
      </manifest>
    </updatecheck>
  </app>
</response>
```

```xml
 <?xml version="1.0" encoding="UTF-8"?>
<response protocol="3.0" server="nebraska">
  <daystart elapsed_days="5434" elapsed_seconds="61945"/>
  <app appid="{87efface-864d-49a5-9bb3-4b050a7c227a}" status="ok">
    <ping status="ok"/>
    <updatecheck status="ok">
      <urls>
        <url codebase="http://192.168.31.120:8080"/>
      </urls>
      <manifest versions="14092.106.2021_11_08_1354">
        <actions>
          <action event="upate" run="delta.bin"/>
          <action ChromeOSVersion="14092.106.2021_11_08_1354" ChromeVersion="1.0.0.0" DisablePayloadBackoff="todo" IsDeltaPayload="todo" MaxDaysToScatter="14" MetadataSignatureRsa="false" MetadataSize="57970" sha256="NbXXg6SyQdXiD+j1/RMqgAfA+sIgFUUPBl6t2+dUgxw=" event="postintall"/>
        </actions>
        <packages>
          <package fp="1.35B5D783A4B241D5E20FE8F5FD132A8007C0FAC22015450F065EADDBE754831C" hash_has256="35B5D783A4B241D5E20FE8F5FD132A8007C0FAC22015450F065EADDBE754831C" name="delta.bin" required="true" size="653474911"/>
        </packages>
      </manifest>
    </updatecheck>
  </app>
</response>
```

second

```xml
<?xml version="1.0" encoding="UTF-8"?>
<response protocol="3.0" server="nebraska">
  <daystart elapsed_days="5425" elapsed_seconds="54494"/>
  <app appid="{87efface-864d-49a5-9bb3-4b050a7c227a}" status="ok">
    <event status="ok"/>
  </app>
</response>

```

third

```xml
<?xml version="1.0" encoding="UTF-8"?>
<response protocol="3.0" server="nebraska">
  <daystart elapsed_days="5425" elapsed_seconds="54551"/>
  <app appid="{87efface-864d-49a5-9bb3-4b050a7c227a}" status="ok">
    <event status="ok"/>
  </app>
</response>

```

fouth

```xml
<?xml version="1.0" encoding="UTF-8"?>
<response protocol="3.0" server="nebraska">
  <daystart elapsed_days="5425" elapsed_seconds="54574"/>
  <app appid="{87efface-864d-49a5-9bb3-4b050a7c227a}" status="ok">
    <event status="ok"/>
  </app>
</response>


```
