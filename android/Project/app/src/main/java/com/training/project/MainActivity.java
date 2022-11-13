package com.training.project;

import androidx.activity.result.ActivityResult;
import androidx.activity.result.ActivityResultCallback;
import androidx.activity.result.ActivityResultLauncher;
import androidx.activity.result.contract.ActivityResultContracts;
import androidx.appcompat.app.AppCompatActivity;

import android.app.Activity;
import android.content.Intent;
import android.os.Bundle;
import android.util.Log;
import android.view.View;

public class MainActivity extends AppCompatActivity {

    String TAG="MainActivity";

    static {
        System.loadLibrary("rust_core");
    }

    LocalVpn vpnRouter = new LocalVpn();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
    }

    public void start(View v){
        Log.d(TAG,"Starting");
        Intent intent = vpnRouter.prepare(this);
        if (intent!=null){
            mStartVpnIntent.launch(intent);
            Log.d(TAG,"Intent started");
        }
        else vpnRouter.start();
    }

    public void stop(View v){
        Log.d(TAG,"Stopping");
        vpnRouter.stop();
    }

    ActivityResultLauncher<Intent> mStartVpnIntent = registerForActivityResult(
            new ActivityResultContracts.StartActivityForResult(),
            new ActivityResultCallback<ActivityResult>() {
                @Override
                public void onActivityResult(ActivityResult result) {
                    if (result.getResultCode() == Activity.RESULT_OK) {
                        Log.d(TAG,"ALL FINALIZED");
                        startService(new Intent(getApplicationContext(),LocalVpn.class));
                    }
                }
            });

}
