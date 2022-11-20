package com.training.project;

import android.net.VpnService;
import android.os.ParcelFileDescriptor;
import android.util.Log;
import android.os.Handler;

public class LocalVpn extends VpnService {

    Handler sleepH = new Handler();
    String TAG="LocalVpn";
    Boolean firstRun=false;
    ParcelFileDescriptor vpnInterface = null;

    @Override
    public void onCreate(){
        Log.d(TAG,"Hello WORLD");
        super.onCreate();
        setupVpn();
        create_native(this);
        start_vpn(vpnInterface.detachFd());
    };

    private void setupVpn() {
        Builder builder = new Builder()
                .addAddress("10.0.2.15", 24)
                .addDnsServer("8.8.8.8")
                .addRoute("0.0.0.0", 0)
                .setSession(TAG);
        vpnInterface = builder.establish();
        Log.d(TAG, "VPN interface has established");
    }

    public void start(){
        Log.d(TAG,"Starting vpn interface");
        stop_vpn();
        setupVpn();
        start_vpn(vpnInterface.detachFd());
    }
    public void stop(){
        Log.d(TAG,"Stopping vpn interface");
        stop_vpn();
    }

    private native void create_native(VpnService vpnInstance);
    private native void destroy_native();
    private native void start_vpn(int fd);
    private native void stop_vpn();

}
